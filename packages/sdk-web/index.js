
import FingerprintJS from '@fingerprintjs/fingerprintjs';


export async function captura_informacoes({host, endpoint="/identity/injest"}) {
  const fp = await FingerprintJS.load();
  const result = await fp.get();

  if(!localStorage.getItem('shaayud_id')){
    const shaayudId = crypto.randomUUID();
    localStorage.setItem('shaayud_id', shaayudId);
  }

  
  const shaayudId = localStorage.getItem('shaayud_id')


  const payload = {
    user_id: 'anonymous',
    shaayud_id:shaayudId,
    // typing_speed: Math.random() * 100,
    fingerprint: result
  };

  console.log('📡 Enviando dados para coleta:', payload);
  const url = new URL(endpoint, host);

  await fetch(url.toString(), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(payload)
  }).then().catch(err=>console.log(err));
  // console.log9()
}