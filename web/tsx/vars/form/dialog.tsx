import { PageAction, PageState } from "../action";
import { VarsForm, VarsInfoForm, VarsParentForm } from "./form";

export const AddVarsFormDialog = () => {
  return (
      <arco.Modal
        title="新增参数"
        visible={PageState.addFormDialogVisible}
        onOk={() => {
          PageState.addFormRef.current?.validate().then((data: any) => {
            PageAction.save(data, () => {
              PageState.setCurrentVarsData(null);
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
        <VarsForm />
      </arco.Modal>
  );
};
export const EditVarsFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="编辑参数"
        visible={PageState.editFormDialogVisible}
        onOk={() => {
          PageState.addFormRef.current?.validate().then((data: any) => {
            PageAction.save(data, () => {
              PageState.setCurrentVarsData(null);
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
        <VarsForm />
      </arco.Modal>
    </>
  );
};
export const InfoVarsFormDialog = () => {
  return (
      <arco.Modal
        title="查看参数"
        visible={PageState.infoFormDialogVisible}
        onOk={() => PageState.setInfoFormDialogVisible(false)}
        onCancel={() => PageState.setInfoFormDialogVisible(false)}
    >
      <VarsInfoForm/>
      </arco.Modal>
  );
};
export const ChangeParentVarsFormDialog = () => {
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
      <VarsParentForm/>
    </arco.Modal>
  );
};

export const ImportVarsFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="导入参数"
        visible={PageState.importFormDialogVisible}
        onOk={() => PageState.setImportFormDialogVisible(false)}
        onCancel={() => PageState.setImportFormDialogVisible(false)}
      ></arco.Modal>
    </>
  );
};
export const ExportVarsFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="导出参数"
        visible={PageState.exportFormDialogVisible}
        onOk={() => PageState.setExportFormDialogVisible(false)}
        onCancel={() => PageState.setExportFormDialogVisible(false)}
      ></arco.Modal>
    </>
  );
};
