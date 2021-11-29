package nodemcu;

@:native("telnet")
@:luaRequire
extern class Telnet {
	public function open(ssid:String, password:String, ?port:Int):Void;
	public function close():Void;
}
