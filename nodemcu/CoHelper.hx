package nodemcu;

@:native("cohelper")
@:luaRequire
@:luaDotMethod
extern class CoHelper {
	public function exec<K, T>(func:(taskYield:(v:T) -> Void, list:Array<K>) -> Void, ...rest:K):T;
}
