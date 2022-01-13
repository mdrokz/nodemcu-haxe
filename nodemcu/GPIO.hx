package nodemcu;

@:native("gpio")
@:luaDotMethod
// for reference - https://nodemcu.readthedocs.io/en/release/modules/gpio/#gpiomode
extern class GPIO {
	static public var OUTPUT:Int;
	static public var INPUT:Int;
	static public var OPENDRAIN:Int;
	static public var INT:Int;
	static public var PULLUP:Int;
	static public var FLOAT:Int;

	public static function mode(pin:Int, mode:Int, ?pullup:Int):Void;
}
