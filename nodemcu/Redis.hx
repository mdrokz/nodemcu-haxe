package nodemcu;

@:native("redis")
@:luaDotMethod
@:luaRequire
extern class Redis {
	public function connnect(host:String, ?port:Int):RedisObject;
}

extern typedef RedisObject = {
	public function subscribe<T>(channel:String, handler:(channel:String, message:T) -> Void):Void;
	public function unsubscribe(channel:String):Void;
	public function publish<T>(channel:String, message:T):Void;
	public function close():Void;
};
