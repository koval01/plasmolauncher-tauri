// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

export type Progress = { type: "bool", content: boolean } | { type: "percentage", content: number } | { type: "bytes", content: [bigint, bigint] } | { type: "count", content: [number, number] } | { type: "indefinate" } | { type: "gamelaunched" } | { type: "none" };