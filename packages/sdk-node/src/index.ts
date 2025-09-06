import { ingest } from "@shaayud/sdk-core";
import geoip from 'geoip-lite'
function normalizeIp(ip: string | undefined): string {
  if (!ip) return "unknown";
  if (ip === "::1") return "127.0.0.1";
  if (ip.startsWith("::ffff:")) return ip.replace("::ffff:", "");
  return ip;
}

function toPlainHeaders(req:any):Record<string,string>{
    const h = req.headers;
    if(!h) return {}
    if(typeof h.entries === "function"){
        return Object.fromEntries(Array.from(h.entries()));
    }
    const out:Record<string, string> = {};
    for(const k of Object.keys(h)){
        const v = (h as any)[k];
        out[k] = Array.isArray(v)?v.join(","):String(v)
    }
    return out
}
const PRIVATE_BLOCKS: RegExp[] = [
    /^10\./,
    /^127\./,
    /^172\.(1[6-9]|2\d|3[0-1])\./,
    /^192\.168\./,
    /^::1$/,
    /^fc00:/,
    /^fe80:/,
    /^::ffff:127\./,
]
function isPrivate(ip:string):boolean {
    return PRIVATE_BLOCKS.some((r)=>r.test(ip))
}
function parseXff(xff:string):string[]{
    return xff.split(",").map((s)=>s.trim()).filter(Boolean)
}
function pickPublicIp(ips:string[]):string | undefined {
    return ips.find((ip)=>!isPrivate(ip))
}
function getPublicClientIp(req:any):string{
    const headers = toPlainHeaders(req); 
    const list:string[]=[];
    const xff = headers["x-forwarded-for"];
    if(xff) list.push(...parseXff(xff));
    if (req.connection?.remoteAddress) list.push(req.connection.remoteAddress);
    if (req.socket?.remoteAddress) list.push(req.socket.remoteAddress);
    if (req.ip) list.push(req.ip);
    const normalized = list.map(normalizeIp).filter(Boolean);
    return pickPublicIp(normalized) || normalized[0] || "unknown";
}
function  geoLookup(ip:string){
    const hit = geoip.lookup(ip);
    if(!hit) return null;
    const [lat, lng] = hit.ll || [null, null]
    return {
        country:hit.country || null, 
        region: hit.region || null, 
        city: hit.city || null, 
        latitude: lat, 
        longitude: lng, 
        timezone: hit.timezone || null
    }
}
function getSessionId(req:any, deviceId:string, tsSec:number):string | undefined {
    const headers = toPlainHeaders(req);
    const fromHeader = headers["x-session-id"] || headers["sessionId"] || headers["session-id"]
    const fromBody = req.body?.session_id || req.body?.sessionId; 
    return fromHeader || fromBody || `sess:${deviceId}:${tsSec}`
}
function getEventId(req:any, identityId:string, sessionId:string, tsSec:number): string | undefined {
    const fromBody = req.body?.event_id || req.body?.eventId; 
    return fromBody || `evt:${identityId}:${sessionId}:${tsSec}`
}
function getEventType(req:any):string|undefined {
    const fromBody = req.body?.event_type || req.body?.eventType; 
    if(fromBody) return fromBody
    const method = req.method || "UNKNOWN";
    const path = req.url || req.originalUrl || ""; 
    return `${method} ${path}`
}
function getFingerprint(req: any): any {
  const fp = req.body?.fingerprint;
  if (!fp) return {};
  if (typeof fp === "string") {
    try { return JSON.parse(fp); } catch { return { raw: fp }; }
  }
  return fp;
}
function getDeviceIdFromFingerprint(fp:any):string | undefined {
  return fp?.id || fp?.visitorId || fp?.deviceId || fp?.device_id;
}
function getIdentityId(req:any):string {
    return req.body?.shaayud_id || req.body?.shaayudId || ""
}
function getUserAget(req:any):string {
    return req.headers?.["user-agent"] || "unknown";
}
function getBackendMeta(req:any){
    const m = req.__backend || {}; 
    return {
        backend_path: m.route || (req.url || req.originalUrl || ""),
        backend_method: m.method || (req.method || "UNKNOWN"),
        backend_host: m.host || (req.headers?.host || "")
    }
}
function getFrontMetaFromBody(req:any){
  return {
    front_url: req.body?.front_url ?? undefined,
    front_path: req.body?.front_path ?? undefined,
    front_referrer: req.body?.front_referrer ?? undefined,
  };
}
export async function ingestPayload(req: any, resp:any){
    const timestamp = new Date();
    const tsIso = timestamp.toISOString();
    const tsSec = Math.floor(timestamp.getTime() / 1000); 

    const shaayudId = getIdentityId(req)
    const fingerprint = getFingerprint(req)
    const deviceId = getDeviceIdFromFingerprint(fingerprint) || "unknown-device"

    const publicIp = getPublicClientIp(req); 
    const geo = geoLookup(publicIp)
    const be = getBackendMeta(req);
    const fe = getFrontMetaFromBody(req);
    const userAgent = getUserAget(req)
    const headersPlain = toPlainHeaders(req); 

    const sessionId = getSessionId(req, deviceId, tsSec)
    const eventId = getEventId(req, shaayudId, sessionId!, tsSec);
    const eventType = getEventType(req)

    const payload = {
        shaayud_id: shaayudId, 
        fingerprint,
        ip:publicIp,
        user_agent:userAgent, 
        header: {
            ...headersPlain,
            "x-client-ip": publicIp,
            "x-geo-country": geo?.country ?? "",
            "x-geo-region": geo?.region ?? "",
            "x-geo-city": geo?.city ?? "",
            "x-geo-timezone": geo?.timezone ?? "",
        },
        timestamp: tsIso, 
        method:req.method || "UNKNOWN", 
        path: req.url || req.originalUrl || "", 

        session_id:sessionId, 
        event_id:eventId, 
        event_type:eventType,

        geo,

        backend_path: be.backend_path,
        backend_method: be.backend_method,
        backend_host: be.backend_host,

        front_url: fe.front_url,
        front_path: fe.front_path,
        front_referrer: fe.front_referrer,
    }

    const res = await ingest(JSON.stringify(payload))
    console.log(res)
}

export function traceBackendRoute(skipPaths: string[] = ["/identity/injest"]) {
  return (req: any, res: any, next: any) => {
    const path = req.url || req.originalUrl || "";
    if (skipPaths.some(p => path.startsWith(p))) return next();
    // anexa campos para o ingestPayload usar
    req.__backend = {
      route: path,                 // backend_path
      method: req.method || "UNKNOWN",
      host: req.headers?.host || "",
    };
    next();
  };
}