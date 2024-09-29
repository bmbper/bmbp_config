import { PageAction, PageState } from "../action";

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
            <arco.Form.Item label="排序" field="dictOrder">
                <arco.InputNumber placeholder="请输入顺序"/>
            </arco.Form.Item>
        </arco.Form>
    );
};
export const DictInfoForm = () => {
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
            <arco.Form.Item label="排序" field="dictOrder">
                <arco.InputNumber placeholder="请输入顺序"/>
            </arco.Form.Item>
        </arco.Form>
    );
};

export const DictParentForm = () => {
    React.useEffect(() => {
        if (PageState.waitChangeDictDataId && PageState.changeParentFormDialogVisible) {
            PageAction.findTreeDataIgnoreDataId(PageState.waitChangeDictDataId);
            PageState.changeParentFormRef?.current.setFieldsValue({ dataId: PageState.waitChangeDictDataId });
        }
    }, [PageState.waitChangeDictDataId]);
    return (
        <arco.Form ref={PageState.changeParentFormRef}>
            <arco.Form.Item label="主键" field="dataId" hidden={true}>
                <arco.Input placeholder=""/>
            </arco.Form.Item>
            <arco.Form.Item label="目标字典" field="dictParentCode" hidden={false} rules={[{required: true, message: "请选择目标字典"}]}>
                <arco.TreeSelect treeData={PageState.parentTreeData}  fieldNames={{
                                                                                        key: 'dataId',
                                                                                        title: 'dictName',
                                                                                        children: 'dictChildren',
                                                                                        }} placeholder="请选择目标字典"/>
            </arco.Form.Item>
        </arco.Form>
    );
};