import { Actor } from "../../../lib_sdk/actors/actor";
import { AssetKind } from "../../../lib_sdk/asset";
import { createActors } from "../../../lib_sdk/create_actors";

setTimeout(function() {
    let alice: Actor;
    let bob: Actor;

    beforeEach(async function() {
        this.timeout(20000);
        const actors = await createActors("e2e-rfc003-btc-eth-refund.log");
        alice = actors.alice;
        bob = actors.bob;
    });

    afterEach(() => {
        if (alice) {
            alice.stop();
        }
        if (bob) {
            bob.stop();
        }
    });

    describe("Bob can refund and then Alice refunds", function() {
        this.timeout(60000);
        it("bitcoin ether", async function() {
            await alice.sendRequest(AssetKind.Bitcoin, AssetKind.Ether);
            await bob.accept();

            await alice.fund();
            await alice.assertAlphaFunded();
            await bob.assertAlphaFunded();

            await bob.fund();
            await alice.assertBetaFunded();
            await bob.assertBetaFunded();

            await bob.refund();
            await alice.refund();

            await alice.assertBetaRefunded();
            await alice.assertAlphaRefunded();
            await bob.assertBetaRefunded();
            await bob.assertAlphaRefunded();

            await bob.assertRefunded();
            await alice.assertRefunded();
        });
    });
    run();
}, 0);
