import { PanelSectionRow, TextField, ButtonItem } from "decky-frontend-lib";
import { VFC } from "react";

interface appProp {
    Subscriptions: Array<any>,
    UpdateSub: any,
    Refresh: Function
}


export const SubList: VFC<appProp> = ({ Subscriptions, UpdateSub, Refresh }) => {
    return (
        <div>
            {
                Subscriptions.map((x) => {
                    return (
                        <div>
                            <ButtonItem label={x.name} description={x.url} onClick={
                                () => {
                                    //删除订阅
                                    UpdateSub((source:Array<any>) => {
                                        let i = source.indexOf(x)
                                        source.splice(i, 1)
                                        return source
                                    });
                                    Refresh()
                                }
                            }>Delete</ButtonItem>
                        </div>
                    );
                })
            }
        </div>
    );
}

