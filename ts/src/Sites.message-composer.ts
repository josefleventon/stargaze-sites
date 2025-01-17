/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { Coin } from "@cosmjs/amino";
import { MsgExecuteContractEncodeObject } from "cosmwasm";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { Decimal, Timestamp, Uint64, InstantiateMsg, CollectionInfoForRoyaltyInfoResponse, RoyaltyInfoResponse, ExecuteMsg, Addr, LayoutContent, NFT, Row, LayoutItem, NFTDisplayType, TextBoxType, Button, SparkDonationType, QueryMsg, Uint128, ParamsResponse, SiteResponse, Site } from "./Sites.types";
export interface SitesMessage {
  contractAddress: string;
  sender: string;
  createSite: ({
    name
  }: {
    name: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  updateInfo: ({
    bio,
    name,
    profileBanner,
    profilePicture
  }: {
    bio?: string;
    name: string;
    profileBanner?: NFT;
    profilePicture?: NFT;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  updateLayout: ({
    layout
  }: {
    layout: Row[];
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class SitesMessageComposer implements SitesMessage {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.createSite = this.createSite.bind(this);
    this.updateInfo = this.updateInfo.bind(this);
    this.updateLayout = this.updateLayout.bind(this);
  }

  createSite = ({
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
          create_site: {
            name
          }
        })),
        funds
      })
    };
  };
  updateInfo = ({
    bio,
    name,
    profileBanner,
    profilePicture
  }: {
    bio?: string;
    name: string;
    profileBanner?: NFT;
    profilePicture?: NFT;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          update_info: {
            bio,
            name,
            profile_banner: profileBanner,
            profile_picture: profilePicture
          }
        })),
        funds
      })
    };
  };
  updateLayout = ({
    layout
  }: {
    layout: Row[];
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          update_layout: {
            layout
          }
        })),
        funds
      })
    };
  };
}