# tag::health[]
@0xca254782cfb5effd;

struct Health @0xdfdf80ca99cd265c {                 # <1>
  # Used as the struct to send out Package Updates
  uuid @0 :Text;
  timestamp @1 :UInt64;
  status @2 :Status;                                # <2>
  msg @3 :Text;
  userId @4 :Text;
  peripherals @5 :List(Peripheral);               # <3>

  enum Status {     # <4>
    green @0;
    red @1;
    yellow @2;
  }

  struct Peripheral {   # <5>
    name @0: Text;
  }
}
# end::health[]

# Our interface to make a  call to process the health update
# tag::interface[]
interface ProcessUpdate @0xeb03883f58bd9352 {     # <1>

  # Interface to update the items
  call @0 (update :Health) -> (passed :Value);    # <2>

  interface Value {                               # <3>
    read @0 () -> (value :Bool);                # <4>
  }
}
# end::interface[]
