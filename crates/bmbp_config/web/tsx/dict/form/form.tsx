import {PageState} from "../action";

export const DictForm = () => {
    React.useEffect(() => {
        if (PageState.currentDictData) {
            PageState.addFormRef.current?.setFieldsValue(PageState.currentDictData);
        }
    }, [PageState.currentDictData]);
    return (
        <arco.Form ref={PageState.addFormRef}>
            <arco.Form.Item label="主键" field="dataId" hidden={true}>
                <arco.Input placeholder=""/>
            </arco.Form.Item>
            <arco.Form.Item label="上级字典编码" field="dictParentCode" hidden={true}>
                <arco.Input placeholder=""/>
            </arco.Form.Item>
            <arco.Form.Item label="字典名称" field="dictName" rules={[{required: true, message: "请输入字典名称"}]}>
                <arco.Input placeholder="请输入字典名称"/>
            </arco.Form.Item>
            <arco.Form.Item label="字典别名" field="dictAlias" rules={[{required: true, message: "请输入字典别名"}]}>
                <arco.Input placeholder="请输入字典别名"/>
            </arco.Form.Item>
            <arco.Form.Item label="字典值" field="dictValue" rules={[{required: true, message: "请输入字典值"}]}>
                <arco.Input placeholder="请输入字典值"/>
            </arco.Form.Item>
            <arco.Form.Item label="排序" field="dataSort">
                <arco.InputNumber placeholder="请输入显示顺序"/>
            </arco.Form.Item>
        </arco.Form>
    );
};
