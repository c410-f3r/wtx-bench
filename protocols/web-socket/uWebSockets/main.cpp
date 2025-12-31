#include <uwebsockets/App.h>

int main() {
    struct PerSocketData {};
    uWS::App().ws<PerSocketData>("/*", {
        .compression = uWS::DISABLED,
        .maxPayloadLength = 100 * 1024 * 1024,
        .idleTimeout = 16,
        .maxBackpressure = 100 * 1024 * 1024,
        .closeOnBackpressureLimit = false,
        .resetIdleTimeoutOnSend = false,
        .sendPingsAutomatically = true,

        .upgrade = nullptr,
        .open = [](auto */*ws*/) {},
        .message = [](auto *ws, std::string_view message, uWS::OpCode opCode) {
            ws->send(message, opCode);
        },
        .drain = [](auto */*ws*/) {},
        .ping = [](auto */*ws*/, std::string_view) {},
        .pong = [](auto */*ws*/, std::string_view) {},
        .close = [](auto */*ws*/, int /*code*/, std::string_view /*message*/) {}
    })
    .listen(9000, [](auto */*listen_socket*/) {})
    .run();
    return 0;
}