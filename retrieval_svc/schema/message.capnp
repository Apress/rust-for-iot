@0xca254782cfb5effd;

struct Health @0xdfdf80ca99cd265c {
  # Used as the struct to send out Package Updates
  uuid @0 :Text;
  timestamp @1 :UInt64;
  status @2 :Status;
  msg @3 :Text;
  userId @4 :Text;
  peripherals @5 :List(Peripheral);

  enum Status {
    green @0;
    red @1;
    yellow @2;
  }

  struct Peripheral {
    name @0: Text;
  }
}

# Our interface to make a  call to process the health update
interface ProcessUpdate @0xeb03883f58bd9352 {

  # Interface to update the items
  call @0 (update :Health) -> (passed :Value);

  interface Value {
    # Wraps a numeric value in an RPC object.  This allows the value
    # to be used in subsequent evaluate() requests without the client
    # waiting for the evaluate() that returns the Value to finish.

    read @0 () -> (value :Bool);
    # Read back the raw numeric value.
  }
}

