import { ingest } from "@shaayud/sdk-core";

export async function ingestPayload(req: any, resp:any){
    const ip =
    req.headers?.['x-forwarded-for']?.split(',')[0]?.trim() ||
    req.connection?.remoteAddress ||
    req.socket?.remoteAddress ||
    req.ip ||
    'unknown';
    //  pub shaayud_id: String,
    // pub fingerprint: JsonValue,
    // pub ip: String,
    // pub user_agent: String,
    // pub timezone: Date,
    const payload = { 
        ip,
        shaayud_id:req.body.shaayud_id,
        fingerprint: req.body.fingerprint,
        user_agent: req.headers?.['user-agent'] || 'unknown',
        header: req.headers || {},
        method: req.method || 'UNKNOWN',
        path: req.url || req.originalUrl || undefined,
        timestamp: new Date().toISOString(),
    }
    
    try{
        await ingest(JSON.stringify(payload))
    }catch(err){
        throw err
    }

}