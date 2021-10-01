package nodemcu;

@:native("bh1750")
@:luaRequire
@:luaDotMethod
extern class BH1750 {
    public function init(sda: Int,scl: Int): Void;
    public function read(): Int;
    public function getlux(): Int;
}