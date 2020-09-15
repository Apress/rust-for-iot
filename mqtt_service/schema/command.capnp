@0xae39671514aa4ef2;

struct Command @0x881697ed2a04b9d4 {
  uuid @0 :Text;
  execute @1 :CommandTypes;

  enum CommandTypes {
    captureImage @0;
    showTemp @1;
  }
}