import {PageState} from "../action";

export const AddDictFormDialog = ()=>{
    const [addFormDialogVisible, setAddFormDialogVisible] = React.useState(false);
    PageState.addFormDialogVisible = addFormDialogVisible;
    PageState.setAddFormDialogVisible = setAddFormDialogVisible;
    return <>
        <arco.Modal
            title='新增字典'
            visible={addFormDialogVisible}
            onOk={() => setAddFormDialogVisible(false)}
            onCancel={() => setAddFormDialogVisible(false)}
        >

        </arco.Modal>
        </>
}
export const EditDictFormDialog = ()=>{
    const [addFormDialogVisible, setAddFormDialogVisible] = React.useState(false);
    PageState.addFormDialogVisible = addFormDialogVisible;
    PageState.setAddFormDialogVisible = setAddFormDialogVisible;
    return <>
        <arco.Modal
            title='编辑字典'
            visible={addFormDialogVisible}
            onOk={() => setAddFormDialogVisible(false)}
            onCancel={() => setAddFormDialogVisible(false)}
        >

        </arco.Modal>
    </>
}
export const InfoDictFormDialog = ()=>{
    const [addFormDialogVisible, setAddFormDialogVisible] = React.useState(false);
    PageState.addFormDialogVisible = addFormDialogVisible;
    PageState.setAddFormDialogVisible = setAddFormDialogVisible;
    return <>
        <arco.Modal
            title='查看字典'
            visible={addFormDialogVisible}
            onOk={() => setAddFormDialogVisible(false)}
            onCancel={() => setAddFormDialogVisible(false)}
        >

        </arco.Modal>
    </>
}
export const ChangeParentDictFormDialog = ()=>{
    const [addFormDialogVisible, setAddFormDialogVisible] = React.useState(false);
    PageState.addFormDialogVisible = addFormDialogVisible;
    PageState.setAddFormDialogVisible = setAddFormDialogVisible;
    return <>
        <arco.Modal
            title='查看字典'
            visible={addFormDialogVisible}
            onOk={() => setAddFormDialogVisible(false)}
            onCancel={() => setAddFormDialogVisible(false)}
        >

        </arco.Modal>
    </>
}
export const ChangeDictShowOrderFormDialog = ()=>{
    const [addFormDialogVisible, setAddFormDialogVisible] = React.useState(false);
    PageState.addFormDialogVisible = addFormDialogVisible;
    PageState.setAddFormDialogVisible = setAddFormDialogVisible;
    return <>
        <arco.Modal
            title='调整顺序'
            visible={addFormDialogVisible}
            onOk={() => setAddFormDialogVisible(false)}
            onCancel={() => setAddFormDialogVisible(false)}
        >

        </arco.Modal>
    </>
}
export const ImportDictFormDialog = ()=>{
    const [addFormDialogVisible, setAddFormDialogVisible] = React.useState(false);
    PageState.addFormDialogVisible = addFormDialogVisible;
    PageState.setAddFormDialogVisible = setAddFormDialogVisible;
    return <>
        <arco.Modal
            title='查看字典'
            visible={addFormDialogVisible}
            onOk={() => setAddFormDialogVisible(false)}
            onCancel={() => setAddFormDialogVisible(false)}
        >

        </arco.Modal>
    </>
}
export const ExportDictFormDialog = ()=>{
    const [addFormDialogVisible, setAddFormDialogVisible] = React.useState(false);
    PageState.addFormDialogVisible = addFormDialogVisible;
    PageState.setAddFormDialogVisible = setAddFormDialogVisible;
    return <>
        <arco.Modal
            title='查看字典'
            visible={addFormDialogVisible}
            onOk={() => setAddFormDialogVisible(false)}
            onCancel={() => setAddFormDialogVisible(false)}
        >

        </arco.Modal>
    </>
}
