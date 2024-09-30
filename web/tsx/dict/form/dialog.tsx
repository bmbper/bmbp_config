import { PageAction, PageState } from "../action";
import { DictForm, DictInfoForm, DictParentForm } from "./form";

export const AddDictFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="新增字典"
        visible={PageState.addFormDialogVisible}
        onOk={() => {
          PageState.addFormRef.current?.validate().then((data: any) => {
            PageAction.save(data, () => {
              PageState.setCurrentDictData(null);
              PageState.setAddFormDialogVisible(false);
              PageState.addFormRef.current?.resetFields();
              PageAction.findTreeData(null);
            });
          });
        }}
        onCancel={() => {
          PageState.addFormRef.current?.resetFields();
          PageState.setAddFormDialogVisible(false);
        }}
      >
        <DictForm />
      </arco.Modal>
    </>
  );
};
export const EditDictFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="编辑字典"
        visible={PageState.editFormDialogVisible}
        onOk={() => {
          PageState.addFormRef.current?.validate().then((data: any) => {
            PageAction.save(data, () => {
              PageState.setCurrentDictData(null);
              PageState.setEditFormDialogVisible(false);
              PageState.addFormRef.current?.resetFields();
              PageAction.findTreeData(null);
            });
          });
        }}
        onCancel={() => {
          PageState.editFormRef.current?.resetFields();
          PageState.setEditFormDialogVisible(false);
        }}
      >
        <DictForm />
      </arco.Modal>
    </>
  );
};
export const InfoDictFormDialog = () => {
  return (
      <arco.Modal
        title="查看字典"
        visible={PageState.infoFormDialogVisible}
        onOk={() => PageState.setInfoFormDialogVisible(false)}
        onCancel={() => PageState.setInfoFormDialogVisible(false)}
    >
      <DictInfoForm/>
      </arco.Modal>
  );
};
export const ChangeParentDictFormDialog = () => {
  return (
      <arco.Modal
        title="变更上级"
        visible={PageState.changeParentFormDialogVisible}
               onOk={() => {
                 PageState.changeParentFormRef.current?.validate().then((data: any) => {
                   debugger;
            PageAction.updateParent(data, () => {
              PageState.setChangeParentFormDialogVisible(false);
              PageState.changeParentFormRef.current?.resetFields();
              PageAction.findTreeData("");
            });
          });
        }}
        onCancel={() => {
          PageState.changeParentFormRef.current?.resetFields();
          PageState.setChangeParentFormDialogVisible(false);
        }}
    >    
      <DictParentForm/>
    </arco.Modal>
  );
};

export const ImportDictFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="查看字典"
        visible={PageState.importFormDialogVisible}
        onOk={() => PageState.setImportFormDialogVisible(false)}
        onCancel={() => PageState.setImportFormDialogVisible(false)}
      ></arco.Modal>
    </>
  );
};
export const ExportDictFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="查看字典"
        visible={PageState.exportFormDialogVisible}
        onOk={() => PageState.setExportFormDialogVisible(false)}
        onCancel={() => PageState.setExportFormDialogVisible(false)}
      ></arco.Modal>
    </>
  );
};
