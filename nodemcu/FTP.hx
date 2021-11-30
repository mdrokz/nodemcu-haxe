package nodemcu;

@:native("ftpserver")
@:luaRequire
// for reference - https://nodemcu.readthedocs.io/en/release/lua-modules/ftpserver/
extern class FTP {
	public function createServer(user: String,pass: String,?dbgFlag: Bool): Void;
    public function open(user: String,pass: String,ssid: String,?dbgFlag: Bool): Void;
    public function close(): Void;
}
