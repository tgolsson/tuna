'use strict';

class TunaConnection {
    constructor (){
        this.vars = new Vars();
        this.connect();
    }

    _recv(msg) {
        let decoded = JSON.parse(msg.data);
        this.vars.addData(decoded);

    }
    _send(msg) {
        let encoded = JSON.stringify(msg, null, 4);
        this.socket.send(encoded);
    }

    listAll() {
        this._send({"ListAll": []});
    }

    connect() {
        let host = location.hostname;
        let port = parseInt(location.port) + 1;

        let socket = new WebSocket(`ws://${host}:${port}`);

        socket.onclose = (ev) => { this.onDisconnect(); };
        socket.onmessage = (msg) => { this._recv(msg); };
        socket.onopen = (_) => { this.onConnect(); };
        socket.onerror = (err) => { console.log(err); };
        this.socket = socket;
    }

    onConnect() {
        console.log(
            "Successfully connected to the game..."
        );

        this.retry_iteration = 0;
        this.listAll();
        this.poll = setInterval(this.listAll.bind(this), 5000);
    }

    cancelAutoUpdate() {
        clearInterval(this.poll);        
    }

    onDisconnect() {
        let random_number_milliseconds = Math.floor(
            Math.random() * 1000
        );
        const maximum_backoff = 64 * 1000;
        let delay = Math.min(
            Math.pow(2, this.retry_iteration) * 1000 +
                random_number_milliseconds,
            maximum_backoff
        );
        console.error(
            "Failed connection to backend, retrying in " +
                Math.floor(delay / 1000) +
                " seconds"
        );
        this.connection = null;
        this.cancelAutoUpdate();
        setTimeout(
            () => this.connect(),
            delay
        );
        this.retry_iteration += 1;
    }

    set(category, name, value) {
        this._send({
            Delta: [[
                category, name, value
            ]]
        });
    }
}


(function() {
    window.tuna = new TunaConnection();
})();
