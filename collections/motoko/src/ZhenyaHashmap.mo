import HashMap "mo:map/Map";
import Nat64 "mo:base/Nat64";
import Hash "mo:base/Hash";
import Iter "mo:base/Iter";
import Option "mo:base/Option";
import Random "random";

actor {
    func f_hash(x : Nat64) : Nat32 = Hash.hash(Nat64.toNat x);
    let hash : HashMap.HashUtils<Nat64> = (f_hash, Nat64.equal, func() = 0);
    stable var map = HashMap.new<Nat64, Nat64>(hash);
    let rand = Random.new(null, 42);

    public func generate(size : Nat32) : async () {
        let rand = Random.new(?size, 1);
        let iter = Iter.map<Nat64, (Nat64, Nat64)>(rand, func x = (x, x));
        map := HashMap.fromIter(iter, hash);
    };
    public query func get_mem() : async (Nat, Nat, Nat) {
        Random.get_memory();
    };
    public func batch_get(n : Nat) : async () {
        for (_ in Iter.range(1, n)) {
            ignore HashMap.get(map, hash, Option.get<Nat64>(rand.next(), 0));
        };
    };
    public func batch_put(n : Nat) : async () {
        for (_ in Iter.range(1, n)) {
            let k = Option.get<Nat64>(rand.next(), 0);
            ignore HashMap.put(map, hash, k, k);
        };
    };
    public func batch_remove(n : Nat) : async () {
        let rand = Random.new(null, 1);
        for (_ in Iter.range(1, n)) {
            ignore HashMap.remove(map, hash, Option.get<Nat64>(rand.next(), 0));
        };
    };
};
