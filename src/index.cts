// This module is the CJS entry point for the library.

// The Rust addon.
import * as addon from "./load.cjs";

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
    function hello(): string;
}

// TODO: Implement JS bindings here

export type Greeting = {
    message: string;
};

export function greeting(): Greeting {
    return { message: "hello" };
}
