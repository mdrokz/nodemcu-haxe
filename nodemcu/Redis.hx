package nodemcu;

@:native("redis")
@:luaDotMethod
@:luaRequire
// for reference - https://nodemcu.readthedocs.io/en/release/lua-modules/redis/
extern class Redis {
	public function connect(host:String, ?port:Int):RedisObject;
}

// for reference - https://nodemcu.readthedocs.io/en/release/lua-modules/redis/#redisconnect
extern typedef RedisObject = {
	public function subscribe<T>(channel:String, handler:(channel:String, message:T) -> Void):Void;
	public function unsubscribe(channel:String):Void;
	public function publish<T>(channel:String, message:T):Void;
	public function close():Void;
};
