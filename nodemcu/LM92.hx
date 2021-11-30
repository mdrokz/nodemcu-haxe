package nodemcu;

@:native("lm92")
@:luaDotMethod
@:luaRequire
// reference - https://nodemcu.readthedocs.io/en/release/lua-modules/lm92/
extern class LM92 {
	public function setup(address:Int):Void;

	public function getTemperature():Int;
	public function getThyst():Int;
	public function getTCrit():Int;
	public function getTLow():Int;
	public function getTHigh():Int;

	public function setThyst(htemp:Int):Void;
	public function setTCrit(ctemp:Int):Void;
	public function setTLow(lwtemp:Int):Void;
	public function setTHigh(hwtemp:Int):Void;

	public function wakeup():Void;
	public function shutDown():Void;
}
