
import FingerprintJS from '@fingerprintjs/fingerprintjs';

const KEY_ID = "shaayud_id";
const KEY_SESS = "shaayud_session_id";

function nowSec(){
  return Math.floor(Date.now() / 1000)
}
function ensureShaayudId(){
  let id = localStorage.getItem(KEY_ID); 
  if(!id){
    id = crypto.randomUUID();
    localStorage.setItem(KEY_ID, id)
  }
  return id
}
function ensureSessionId(deviceId){
  let sid = sessionStorage.getItem(KEY_SESS) || localStorage.getItem(KEY_SESS);
  if(!sid){
    sid = `sess:${deviceId}:${nowSec()}`;
    sessionStorage.setItem(KEY_SESS, sid)
  }
  return sid
}

export async function captura_informacoes({host, endpoint="/identity/injest", eventType = "page_view"}) {
  const fp = await FingerprintJS.load();
  const result = await fp.get();
  const deviceId = result.visitorId || "unknown-device"

  const shaayudId = ensureShaayudId();
  const sessionId = ensureSessionId(deviceId); 
  const tsSec = nowSec();

  const eventId = `evt:${shaayudId}:${sessionId}:${tsSec}`

  const frontUrl  = typeof location !== "undefined" ? location.href : "";
  const frontPath = typeof location !== "undefined" ? location.pathname + location.search : "";
  const referrer  = typeof document !== "undefined" ? document.referrer : "";


  const payload = {
    user_id: 'anonymous',
    shaayud_id:shaayudId,
    // typing_speed: Math.random() * 100,
    fingerprint: result,
    session_id:sessionId, 
    event_id:eventId,
    event_type: eventType,

    front_url: frontUrl,
    front_path: frontPath,
    front_referrer: referrer,
  };

  console.log('📡 Enviando dados para coleta:', payload);
  const url = new URL(endpoint, host);

  await fetch(url.toString(), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json', 
      "x-session-id":sessionId
    },
    body: JSON.stringify(payload), 
    keepalive:true,
  }).then().catch(err=>console.log(err));
  // console.log9()
}