import * as schinese from "./schinese.json";
import * as tchinese from "./tchinese.json";
import * as english from "./english.json";
import * as german from "./german.json";
import * as japanese from "./japanese.json";
import * as koreana from "./koreana.json";
import * as thai from "./thai.json";
import * as bulgarian from "./bulgarian.json";
import * as italian from "./italian.json";
import * as french from "./french.json";


export interface LanguageProps {
  label: string;
  strings: any;
  credit: string[];
  locale: string;
}

export const defaultLanguage = "english";
export const defaultLocale = "en";
export const defaultMessages = english;

export const localizeMap: { [key: string]: LanguageProps } = {
  schinese: {
    label: "简体中文",
    strings: schinese,
    credit: ["yxx"],
    locale: "zh-CN",
  },
  tchinese: {
    label: "繁體中文",
    strings: tchinese,
    credit: [],
    locale: "zh-TW",
  },
  english: {
    label: "English",
    strings: english,
    credit: [],
    locale: "en",
  },
  german: {
    label: "Deutsch",
    strings: german,
    credit: ["dctr"],
    locale: "de",
  },
  japanese: {
    label: "日本語",
    strings: japanese,
    credit: [],
    locale: "ja",
  },
  koreana: {
    label: "한국어",
    strings: koreana,
    credit: [],
    locale: "ko",
  },
  thai: {
    label: "ไทย",
    strings: thai,
    credit: [],
    locale: "th",
  },
  bulgarian: {
    label: "Български",
    strings: bulgarian,
    credit: [],
    locale: "bg",
  },
  italian: {
    label: "Italiano",
    strings: italian,
    credit: [],
    locale: "it",
  },
  french: {
    label: "Français",
    strings: french,
    credit: [],
    locale: "fr",
  },
};

export enum localizeStrEnum {
  SERVICE = "SERVICE",
  TOOLS = "TOOLS",
  VERSION = "VERSION",
  ABOUT = "ABOUT",
  DEBUG = "DEBUG",

  // Subscriptions manager
  SUBSCRIPTIONS = "SUBSCRIPTIONS",
  SUBSCRIPTIONS_LINK = "SUBSCRIPTIONS_LINK",
  SELECT_SUBSCRIPTION = "SELECT_SUBSCRIPTION",
  DOWNLOAD = "DOWNLOAD",
  UPDATE_ALL = "UPDATE_ALL",
  DELETE = "DELETE",

  // QAM
  ENABLE_CLASH = "ENABLE_CLASH",
  ENABLE_CLASH_DESC = "ENABLE_CLASH_DESC",
  ENABLE_CLASH_FAILED = "ENABLE_CLASH_FAILED",
  ENABLE_CLASH_LOADING = "ENABLE_CLASH_LOADING",
  ENABLE_CLASH_IS_RUNNING = "ENABLE_CLASH_IS_RUNNING",
  MANAGE_SUBSCRIPTIONS = "MANAGE_SUBSCRIPTIONS",
  OPEN_DASHBOARD = "OPEN_DASHBOARD",
  SELECT_DASHBOARD = "SELECT_DASHBOARD",
  ALLOW_REMOTE_ACCESS = "ALLOW_REMOTE_ACCESS",
  ALLOW_REMOTE_ACCESS_DESC = "ALLOW_REMOTE_ACCESS_DESC",
  SKIP_PROXY = "SKIP_PROXY",
  SKIP_PROXY_DESC = "SKIP_PROXY_DESC",
  OVERRIDE_DNS = "OVERRIDE_DNS",
  OVERRIDE_DNS_DESC = "OVERRIDE_DNS_DESC",
  ENHANCED_MODE = "ENHANCED_MODE",
  ENHANCED_MODE_DESC = "ENHANCED_MODE_DESC",
  RESTART_CORE = "RESTART_CORE",
  RESET_NETWORK = "RESET_NETWORK",
  REINSTALL_PLUGIN = "REINSTALL_PLUGIN",
  UPDATE_TO = "UPDATE_TO",
  INSTALLED_VERSION = "INSTALLED_VERSION",
  LATEST_VERSION = "LATEST_VERSION",
}
