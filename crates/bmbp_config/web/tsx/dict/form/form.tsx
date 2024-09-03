export const DictAddForm = () => {
    return (<arco.Form>
            <arco.Form.Item label='字典名称' name='dictName' rules={[{required: true, message: '请输入字典名称'}]}>
                <arco.Input placeholder='请输入字典名称'/>
            </arco.Form.Item>
            <arco.Form.Item label='字典编码' name='dictCode' rules={[{required: true, message: '请输入字典编码'}]}>
                <arco.Input placeholder='请输入字典编码'/>
            </arco.Form.Item>
        </arco.Form>
    )
}
