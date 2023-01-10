"use strict";
exports.__esModule = true;
exports.Native = exports.NetworkId = exports.ChainName = exports.ChainId = void 0;
var ChainId;
(function (ChainId) {
    ChainId[ChainId["BIFROST"] = 2001] = "BIFROST";
    ChainId[ChainId["MOONBEAM"] = 2004] = "MOONBEAM";
    ChainId[ChainId["ASTAR"] = 2006] = "ASTAR";
    ChainId[ChainId["MOONRIVER"] = 2023] = "MOONRIVER";
})(ChainId = exports.ChainId || (exports.ChainId = {}));
var ChainName;
(function (ChainName) {
    ChainName["BIFROST"] = "bifrost";
    ChainName["MOONBEAM"] = "moonbeam";
    ChainName["ASTAR"] = "astar";
    ChainName["MOONRIVER"] = "moonriver";
})(ChainName = exports.ChainName || (exports.ChainName = {}));
var NetworkId;
(function (NetworkId) {
    NetworkId[NetworkId["KUSAMA"] = 200] = "KUSAMA";
    NetworkId[NetworkId["POLKADOT"] = 300] = "POLKADOT";
})(NetworkId = exports.NetworkId || (exports.NetworkId = {}));
var Native;
(function (Native) {
    Native["N"] = "N";
    Native["L"] = "L";
    Native["P"] = "P";
})(Native = exports.Native || (exports.Native = {}));
