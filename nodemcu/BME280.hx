package nodemcu;

import haxe.extern.EitherType;

@:multiReturn
extern class Reading {
	public var T:Float;
	public var P:Float;
	public var H:Float;
	public var QNH:Float;
}

@:native("bme280")
@:luaRequire
@:luaDotMethod
extern class BME280 {
	public function setup(id:Int, ?address:Int, ?temp_oss:Int, ?press_oss:Int, ?humi_oss:Int, ?sensor_mode:Int, ?inactive_duration:Int, ?IIR_filter:Int,
		?cold_start:Int):BMEObject;
}

extern typedef BMEObject = {
	public function setup(id:Int, ?address:Int, ?temp_oss:Int, ?press_oss:Int, ?humi_oss:Int, ?sensor_mode:Int, ?inactive_duration:Int, ?IIR_filter:Int,
		?cold_start:Int):BMEObject;

	public function altitude(P:Float, QNH:Float):Float;
	public function dewpoint(H:Float, T:Float):Float;
	public function qfe2qnh(P:Float, altitude:Float):Float;
	public function read(?altitude:Array<Float>):EitherType<Void, Reading>;
	public function startReadout(delay:Int, callback:(?T:Float, ?P:Float, ?H:Float, ?QNH:Float) -> Void):Void;
}
