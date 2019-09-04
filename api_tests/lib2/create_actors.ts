import { Logger } from "log4js";
import { ALICE_CONFIG, BOB_CONFIG } from "../lib/config";
import { Actors } from "./actors";
import { Actor } from "./actors/actor";

export async function createActors(
    loggerFactory: () => Logger
): Promise<Actors> {
    const alice = new Actor(
        loggerFactory,
        "alice",
        `http://localhost:${ALICE_CONFIG.httpApiPort}`
    );

    const bob = new Actor(
        loggerFactory,
        "bob",
        `http://localhost:${BOB_CONFIG.httpApiPort}`
    );

    const actors = new Actors(
        new Map<string, Actor>([["alice", alice], ["bob", bob]])
    );

    alice.actors = actors;
    bob.actors = actors;

    return Promise.resolve(actors);
}
