package nodemcu;

@:native("HDC1000")
@:luaDotMethod
@:luaRequire
extern class HDC1000 {
	public function setup(drdyn:Int):Void;
	public function config(address:Int, resolution:Int, heater:Int):Void;
	public function getTemp():Int;
	public function getHumi():Float;
	public function batteryDead():Bool;
}
