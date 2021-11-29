package nodemcu;

import haxe.extern.EitherType;

@:native("yeelink")
@:luaDotMethod
@:luaRequire
extern class YeeLink {
	public function init(device:Int, sensor:Int, apiKey:String):EitherType<String, Bool>;
	public function getDNS():EitherType<String, Void>;
	public function update<T>(datapoint:T):Void;
}
