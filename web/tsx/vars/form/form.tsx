import { PageAction, PageState } from "../action";

export const VarsForm = () => {
    React.useEffect(() => {
        if (PageState.currentVarsData) {
            PageState.addFormRef.current?.setFieldsValue(PageState.currentVarsData);
        }
    }, [PageState.currentVarsData]);
    return (
        <arco.Form ref={PageState.addFormRef}>
            <arco.Form.Item label="主键" field="dataId" hidden={true}>
                <arco.Input placeholder=""/>
            </arco.Form.Item>
            <arco.Form.Item label="上级参数编码" field="varsParentCode" hidden={true}>
                <arco.Input placeholder=""/>
            </arco.Form.Item>
            <arco.Form.Item label="参数名称" field="varsName" rules={[{required: true, message: "请输入参数名称"}]}>
                <arco.Input placeholder="请输入参数名称"/>
            </arco.Form.Item>
            <arco.Form.Item label="参数编码" field="varsCode" rules={[{required: true, message: "请输入参数编码"}]}>
                <arco.Input placeholder="请输入参数别名"/>
            </arco.Form.Item>
            <arco.Form.Item label="参数值" field="varsValue" >
                <arco.Input placeholder="请输入参数值"/>
            </arco.Form.Item>
            <arco.Form.Item label="排序" field="varsOrder">
                <arco.InputNumber placeholder="请输入顺序"/>
            </arco.Form.Item>
        </arco.Form>
    );
};
export const VarsInfoForm = () => {
    React.useEffect(() => {
        if (PageState.currentVarsData) {
            PageState.addFormRef.current?.setFieldsValue(PageState.currentVarsData);
        }
    }, [PageState.currentVarsData]);
    return (
        <arco.Form ref={PageState.addFormRef}>
            <arco.Form.Item label="主键" field="dataId" hidden={true}>
                <arco.Input placeholder=""/>
            </arco.Form.Item>
            <arco.Form.Item label="上级参数编码" field="varsParentCode" hidden={true}>
                <arco.Input placeholder=""/>
            </arco.Form.Item>
            <arco.Form.Item label="参数名称" field="varsName" rules={[{required: true, message: "请输入参数名称"}]}>
                <arco.Input placeholder="请输入参数名称"/>
            </arco.Form.Item>
            <arco.Form.Item label="参数编码" field="varsCode" rules={[{required: true, message: "请输入参数编码"}]}>
                <arco.Input placeholder="请输入参数编码"/>
            </arco.Form.Item>
            <arco.Form.Item label="参数值" field="varsValue">
                <arco.Input placeholder="请输入参数值"/>
            </arco.Form.Item>
            <arco.Form.Item label="排序" field="varsOrder">
                <arco.InputNumber placeholder="请输入顺序"/>
            </arco.Form.Item>
        </arco.Form>
    );
};

export const VarsParentForm = () => {
    React.useEffect(() => {
        if (PageState.waitChangeVarsDataId && PageState.changeParentFormDialogVisible) {
            PageAction.findTreeDataIgnoreDataId(PageState.waitChangeVarsDataId);
            PageState.changeParentFormRef?.current.setFieldsValue({ dataId: PageState.waitChangeVarsDataId });
        }
    }, [PageState.waitChangeVarsDataId]);
    return (
        <arco.Form ref={PageState.changeParentFormRef}>
            <arco.Form.Item label="主键" field="dataId" hidden={true}>
                <arco.Input placeholder=""/>
            </arco.Form.Item>
            <arco.Form.Item label="目标参数" field="varsParentCode" hidden={false}>
                <arco.TreeSelect treeData={PageState.parentTreeData}  fieldNames={{
                                                                                        key: 'varsCode',
                                                                                        title: 'varsName',
                                                                                        children: 'varsChildren',
                                                                                        }} placeholder="请选择目标参数"/>
            </arco.Form.Item>
        </arco.Form>
    );
};