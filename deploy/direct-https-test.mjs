import fs from 'node:fs';
import https from 'node:https';

const options = {
  key: fs.readFileSync('/etc/letsencrypt/live/iie.cskaoyan.cn/privkey.pem'),
  cert: fs.readFileSync('/etc/letsencrypt/live/iie.cskaoyan.cn/fullchain.pem'),
  minVersion: 'TLSv1.2'
};

const body = '<!doctype html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Direct HTTPS Test</title></head><body><h1>Direct HTTPS OK</h1><p>This response does not use Nginx, Svelte, Rust, or a database.</p></body></html>';

const server = https.createServer(options, (request, response) => {
  console.log(JSON.stringify({
    time: new Date().toISOString(),
    address: request.socket.remoteAddress,
    method: request.method,
    url: request.url,
    userAgent: request.headers['user-agent'] ?? ''
  }));

  response.writeHead(200, {
    'Content-Type': 'text/html; charset=utf-8',
    'Content-Length': Buffer.byteLength(body),
    'Cache-Control': 'no-store, no-cache, must-revalidate',
    Connection: 'close'
  });
  response.end(body);
});

server.on('tlsClientError', (error, socket) => {
  console.error(JSON.stringify({
    time: new Date().toISOString(),
    address: socket.remoteAddress,
    tlsError: error.message
  }));
});

server.listen(443, () => {
  console.log('Direct HTTPS test listening on port 443');
});
