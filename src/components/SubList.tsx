import { ButtonItem } from "@decky/ui";
import { FC } from "react";
import * as backend from "../backend/backend";
interface appProp {
  Subscriptions: Array<any>;
  UpdateSub: any;
  Refresh: Function;
}

export const SubList: FC<appProp> = ({ Subscriptions, UpdateSub, Refresh }) => {
  return (
    <div>
      {Subscriptions.map((x) => {
        return (
          <div>
            <ButtonItem
              label={x.name}
              description={x.url}
              onClick={() => {
                //删除订阅
                UpdateSub((source: Array<any>) => {
                  let i = source.indexOf(x);
                  source.splice(i, 1);
                  return source;
                });
                backend.resolve(backend.deleteSub(x.id), () => {});
                Refresh();
              }}
            >
              Delete
            </ButtonItem>
          </div>
        );
      })}
    </div>
  );
};
