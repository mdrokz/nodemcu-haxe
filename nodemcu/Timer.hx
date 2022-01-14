package nodemcu;

@:native("tmr")
@:luaDotMethod
// for reference - https://nodemcu.readthedocs.io/en/release/modules/tmr/
extern class Timer {
    public static function delay(us: Float): Void;
}
