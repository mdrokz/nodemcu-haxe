package nodemcu;

@:native("bh1750")
@:luaRequire
@:luaDotMethod
// for reference - https://nodemcu.readthedocs.io/en/release/lua-modules/bh1750/
extern class BH1750 {
    public function init(sda: Int,scl: Int): Void;
    public function read(): Int;
    public function getlux(): Int;
}