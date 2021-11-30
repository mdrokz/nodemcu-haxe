package nodemcu;

@:native("cohelper")
@:luaRequire
@:luaDotMethod
// for reference - https://nodemcu.readthedocs.io/en/release/lua-modules/cohelper/
extern class CoHelper {
	public function exec<K, T>(func:(taskYield:(v:T) -> Void, list:Array<K>) -> Void, ...rest:K):T;
}
