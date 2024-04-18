import { WebSocketServer } from 'ws';

const ws = new WebSocketServer({ port: 9000 });

ws.on('connection', (ws) => {
	ws.on('error', (e) => console.error(e));
	ws.on('message', (data) => {
		ws.send(data);
	});
});