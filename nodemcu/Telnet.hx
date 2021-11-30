package nodemcu;

@:native("telnet")
@:luaRequire
// for reference - https://nodemcu.readthedocs.io/en/release/lua-modules/telnet/
extern class Telnet {
	public function open(ssid:String, password:String, ?port:Int):Void;
	public function close():Void;
}
