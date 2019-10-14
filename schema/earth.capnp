@0xd286dcc8ab9257f9;

struct Planet {
	nodes @0 :Map(Node, Map(Text, Text));
	ways @1 :List(Way);
	relations @2 :List(Rel);
}

struct Node {
	id @0 :Float64;
}

struct Way {
	nodes @0 :List(Node);
	tags @1 :Map(Text, Text);
}

struct Rel {}

struct Map(Key, Value) {
	entries @0 :List(Entry);
	struct Entry {
		key @0 :Key;
		value @1 :Value;
	}
}
