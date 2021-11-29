package nodemcu;

import lua.Table;

@:native("ds18b20")
@:luaRequire
extern class DS18B20 {
	public var C:String;
	public var F:String;
	public var temp:Table<Int, String>;
	public var sens:Table<Int, Int>;
	public function enable_debug():Void;
	public function read_temp(callback:(temp:Table<Int, String>, pin:Int, ?unit:String, ?force_search:Bool, ?save_search:Bool) -> Void):Void;
}
