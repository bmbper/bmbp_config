import {PageState} from "../action";
import {DictAddForm} from "./form";

export const AddDictFormDialog = () => {
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
            <DictAddForm/>
        </arco.Modal>
    </>
}
export const EditDictFormDialog = () => {
    const [editFormDialogVisible, setEditFormDialogVisible] = React.useState(false);
    PageState.editFormDialogVisible = editFormDialogVisible;
    PageState.setEditFormDialogVisible = setEditFormDialogVisible;
    return <>
        <arco.Modal
            title='编辑字典'
            visible={editFormDialogVisible}
            onOk={() => setEditFormDialogVisible(false)}
            onCancel={() => setEditFormDialogVisible(false)}
        >

        </arco.Modal>
    </>
}
export const InfoDictFormDialog = () => {
    const [infoFormDialogVisible, setInfoFormDialogVisible] = React.useState(false);
    PageState.infoFormDialogVisible = infoFormDialogVisible;
    PageState.setInfoFormDialogVisible = setInfoFormDialogVisible;
    return <>
        <arco.Modal
            title='查看字典'
            visible={infoFormDialogVisible}
            onOk={() => setInfoFormDialogVisible(false)}
            onCancel={() => setInfoFormDialogVisible(false)}
        >

        </arco.Modal>
    </>
}
export const ChangeParentDictFormDialog = () => {
    const [changeParentFormDialogVisible, setChangeParentFormDialogVisible] = React.useState(false);
    PageState.changeParentFormDialogVisible = changeParentFormDialogVisible;
    PageState.setChangeParentFormDialogVisible = setChangeParentFormDialogVisible;
    return <>
        <arco.Modal
            title='查看字典'
            visible={changeParentFormDialogVisible}
            onOk={() => setChangeParentFormDialogVisible(false)}
            onCancel={() => setChangeParentFormDialogVisible(false)}
        >

        </arco.Modal>
    </>
}
export const ChangeDictShowOrderFormDialog = () => {
    const [changeShowOrderFormDialogVisible, setChangeShowOrderFormDialogVisible] = React.useState(false);
    PageState.changeShowOrderFormDialogVisible = changeShowOrderFormDialogVisible;
    PageState.setChangeShowOrderFormDialogVisible = setChangeShowOrderFormDialogVisible;
    return <>
        <arco.Modal
            title='调整顺序'
            visible={changeShowOrderFormDialogVisible}
            onOk={() => setChangeShowOrderFormDialogVisible(false)}
            onCancel={() => setChangeShowOrderFormDialogVisible(false)}
        >

        </arco.Modal>
    </>
}
export const ImportDictFormDialog = () => {
    const [importFormDialogVisible, setImportFormDialogVisible] = React.useState(false);
    PageState.importFormDialogVisible = importFormDialogVisible;
    PageState.setImportFormDialogVisible = setImportFormDialogVisible;
    return <>
        <arco.Modal
            title='查看字典'
            visible={importFormDialogVisible}
            onOk={() => setImportFormDialogVisible(false)}
            onCancel={() => setImportFormDialogVisible(false)}
        >

        </arco.Modal>
    </>
}
export const ExportDictFormDialog = () => {
    const [exportFormDialogVisible, setExportFormDialogVisible] = React.useState(false);
    PageState.exportFormDialogVisible = exportFormDialogVisible;
    PageState.setExportFormDialogVisible = setExportFormDialogVisible;
    return <>
        <arco.Modal
            title='查看字典'
            visible={exportFormDialogVisible}
            onOk={() => setExportFormDialogVisible(false)}
            onCancel={() => setExportFormDialogVisible(false)}
        >
        </arco.Modal>
    </>
}
