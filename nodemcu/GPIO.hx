package nodemcu;

@:native("gpio")
@:luaDotMethod
// for reference - https://nodemcu.readthedocs.io/en/release/modules/gpio/#gpiomode
extern class GPIO {
	extern public static final OUTPUT:Int;
	extern public static final INPUT:Int;
	extern public static final OPENDRAIN:Int;
	extern public static final INT:Int;
	extern public static final PULLUP:Int;
	extern public static final FLOAT:Int;
	extern public static final HIGH:Int;
	extern public static final LOW:Int;

	public static function mode(pin:Int, mode:Int, ?pullup:Int):Void;

	public static function read(pin:Int):Bool;

	public static function write(pin:Int, level:Int): Void;
}
