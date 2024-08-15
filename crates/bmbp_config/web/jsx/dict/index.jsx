import {DictAction} from "./action";

window.onload = () => {
    const root = ReactDOM.createRoot(document.getElementById("app"));
    root.render(<DictView/>);
}
const DictView = () => {
    return (
        <div className="bmbp-app-fluid">
            <arco.Grid.Row guides={[1, 1]} style={{height: '100vh'}}>
                <arco.Grid.Col flex={'260px'}>
                    <DictTreeView/>
                </arco.Grid.Col>
                <arco.Divider type="vertical" style={{height: '100%'}}>
                </arco.Divider>
                <arco.Grid.Col flex={'auto'} style={{height: '100%'}}>
                    <DictGridView/>
                </arco.Grid.Col>
            </arco.Grid.Row>
        </div>
    );
};


const DictTreeView = () => {
    const treeData = [
        {
            title: '一级 1',
            key: '0-0',
            children: [
                {
                    title: '二级 1-0',
                    key: '0-0-0',
                    children: [
                        {
                            title: '三级 1-0-0',
                            key: '0-0-0-0',
                        },
                        {
                            title: '三级 1-0-1',

                        }
                    ]
                }
            ]
        }
    ];
    const DictTreeNodeMoreAction = () => {
        return (<arco.Menu style={{width: '100px', background: '#fff', border: '1px solid #e8e8e8'}}>
            <arco.Menu.Item key="1" onClick={() => {
                DictAction.addDict();
            }}>新增同级
            </arco.Menu.Item>
            <arco.Menu.Item key="2" onClick={() => {
                DictAction.addDict();
            }}>新增子级
            </arco.Menu.Item>
            <arco.Menu.Item key="3" onClick={() => {
                DictAction.addDict();
            }}>编辑
            </arco.Menu.Item>
            <arco.Menu.Item key="4" onClick={() => {
                DictAction.addDict();
            }}>删除
            </arco.Menu.Item>
            <arco.Menu.Item key="5" onClick={() => {
                DictAction.addDict();
            }}>启用
            </arco.Menu.Item>
            <arco.Menu.Item key="6" onClick={() => {
                DictAction.addDict();
            }}>停用
            </arco.Menu.Item>
            <arco.Menu.Item key="7" onClick={() => {
                DictAction.addDict();
            }}>变更父级
            </arco.Menu.Item>
        </arco.Menu>);
    };
    const renderTreeNodExtra = (node) => {
        return (
            <arco.Dropdown droplist={<DictTreeNodeMoreAction/>} position='bl'>
                <arcoicon.IconMore
                    style={{
                        position: 'absolute',
                        right: 8,
                        fontSize: 12,
                        top: 10,
                        color: '#3370ff',
                    }}
                />
            </arco.Dropdown>
        );
    };
    return (<div>
        <div style={{display: 'block'}}>
            <arco.Input placeholder="请输入" addAfter={<arcoicon.IconSearch/>}/>
        </div>
        <arco.Tree treeData={treeData} blockNode={true} renderExtra={renderTreeNodExtra}></arco.Tree>
    </div>)
}

const DictGridView = () => {
    return (<div className="bmbp-grid-container"><DictGridSearchForm/><DictGridToolBar/><DictGridTable/><DictGridPage/>
    </div>)
}
const DictGridSearchForm = () => {
    return (<div className="bmbp-grid-search">Grid</div>)
}
const DictGridToolBar = () => {
    return (<div className="bmbp-grid-toolbar">
        <arco.Button type="primary">新增</arco.Button>
    </div>)
}
const DictGridTable = () => {
    return (<div className="bmbp-grid-table">Grid</div>)
}
const DictGridPage = () => {
    const [size, setSize] = React.useState('default');
    return (<div className="bmbp-grid-page">
        <arco.Pagination size={size} total={50} showTotal showJumper sizeCanChange /></div>)
}
