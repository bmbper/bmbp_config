export const DictAddForm = () => {
  return (
    <arco.Form>
      <arco.Form.Item label="上级字典编码" name="dictParentCode" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="字典名称"
        name="dictName"
        rules={[{ required: true, message: "请输入字典名称" }]}
      >
        <arco.Input placeholder="请输入字典名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="字典别名"
        name="dictAlias"
        rules={[{ required: true, message: "请输入字典别名" }]}
      >
        <arco.Input placeholder="请输入字典别名" />
      </arco.Form.Item>
      <arco.Form.Item
        label="字典值"
        name="dictValue"
        rules={[{ required: true, message: "请输入字典值" }]}
      >
        <arco.Input placeholder="请输入字典值" />
      </arco.Form.Item>
      <arco.Form.Item label="字典显示顺序" name="dataSort">
        <arco.Input.Number placeholder="请输入字典显示顺序" />
      </arco.Form.Item>
    </arco.Form>
  );
};
