@0x98802735337e2518;

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


