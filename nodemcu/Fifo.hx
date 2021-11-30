package nodemcu;

@:native("fifo")
@:luaDotMethod
@:luaRequire
// for reference - https://nodemcu.readthedocs.io/en/release/lua-modules/fifo/
extern class Fifo {
	@:native("new")
	public function new_():FifoObject;
}

extern typedef FifoObject = {
	public function dequeue<K, T>(k:(v:K) -> T):Bool;
	public function queue<K, T>(a:K, ?k:(v:K) -> T):Void;
}
