import { defaultLocale, localizeMap, LocalizeStrKey } from "./localizeMap";

import i18n, { Resource } from "i18next";

export class localizationManager {
  private static language = "english";

  public static async init() {
    const language =
      (await SteamClient.Settings.GetCurrentLanguage()) || "english";
    this.language = language;
    console.log(">>>>>>>>>> Language: " + this.language);

    const resources: Resource = Object.keys(localizeMap).reduce(
      (acc: Resource, key) => {
        acc[localizeMap[key].locale] = {
          translation: localizeMap[key].strings,
        };
        return acc;
      },
      {}
    );

    i18n.init({
      resources: resources,
      lng: this.getLocale(), // 目标语言
      fallbackLng: defaultLocale, // 回落语言
      returnEmptyString: false, // 空字符串不返回, 使用回落语言
      interpolation: {
        escapeValue: false,
      },
    });
  }

  private static getLocale() {
    return localizeMap[this.language]?.locale ?? defaultLocale;
  }

  public static getString(
    defaultString: LocalizeStrKey,
    variables?: Record<string, unknown>
  ) {
    console.log(">>>>>>>>>> getString: " + defaultString);
    return i18n.t(defaultString, variables);
  }
}
