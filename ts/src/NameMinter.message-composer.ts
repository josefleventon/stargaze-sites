/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { Coin } from "@cosmjs/amino";
import { MsgExecuteContractEncodeObject } from "cosmwasm";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { Uint128, InstantiateMsg, ExecuteMsg, QueryMsg, AdminResponse, CollectionResponse, Decimal, ParamsResponse, SudoParams, Addr, WhitelistsResponse } from "./NameMinter.types";
export interface NameMinterMessage {
  contractAddress: string;
  sender: string;
  mintAndList: ({
    name
  }: {
    name: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  updateAdmin: ({
    admin
  }: {
    admin?: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  pause: ({
    pause
  }: {
    pause: boolean;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  addWhitelist: ({
    address
  }: {
    address: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  removeWhitelist: ({
    address
  }: {
    address: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class NameMinterMessageComposer implements NameMinterMessage {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.mintAndList = this.mintAndList.bind(this);
    this.updateAdmin = this.updateAdmin.bind(this);
    this.pause = this.pause.bind(this);
    this.addWhitelist = this.addWhitelist.bind(this);
    this.removeWhitelist = this.removeWhitelist.bind(this);
  }

  mintAndList = ({
    name
  }: {
    name: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          mint_and_list: {
            name
          }
        })),
        funds
      })
    };
  };
  updateAdmin = ({
    admin
  }: {
    admin?: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          update_admin: {
            admin
          }
        })),
        funds
      })
    };
  };
  pause = ({
    pause
  }: {
    pause: boolean;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          pause: {
            pause
          }
        })),
        funds
      })
    };
  };
  addWhitelist = ({
    address
  }: {
    address: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          add_whitelist: {
            address
          }
        })),
        funds
      })
    };
  };
  removeWhitelist = ({
    address
  }: {
    address: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          remove_whitelist: {
            address
          }
        })),
        funds
      })
    };
  };
}