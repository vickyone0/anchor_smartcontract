import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyAnchorProject } from "../target/types/my_anchor_project";
import { ComplexCounterAndUserProfile } from "../target/types/complex_counter_and_user_profile";

describe("my_anchor_project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.myAnchorProject as Program<ComplexCounterAndUserProfile>;

  it("Is initialized!", async () => {
    const message = "Hello from the test!";

    // Add your test here.
    const tx = await program.methods.sayHello(message).rpc();
    console.log("Your transaction signature", tx);
  });
});
