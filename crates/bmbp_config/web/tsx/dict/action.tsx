// 全局状态声明
interface PageStateModel {
    selectedRowKeys?: any;
    selectedRows?: any;
    setSelectedRowKeys?: any;
    setSelectedRows?: any;
    treeData?: any;
    setTreeData?: any;
    setSelectTreeNodeData?: any;
    selectTreeNodeData?: any;
    selectTreeNodeKeys?: any;
    setSelectTreeNodeKeys?: any;
    setSearchFormData?: any;
    searchFormData?: any;
    searchFormRef?: any;
    gridData?: any;
    setGridData?: any;
    showPageSize?: any;
    setShowPageSize?: any;
    pageData?: any;
    setPageData?: any;

}

// 全局状态
export const PageState: PageStateModel = {}

// 全局方法
export const PageAction = {
    init: (props: any) => {
        // 字典树数据
        const [treeData, setTreeData] = React.useState([]);
        PageState.treeData = treeData;
        PageState.setTreeData = setTreeData;

        // 字典树选中节点数据
        const [selectTreeNodeData, setSelectTreeNodeData] = React.useState();
        PageState.selectTreeNodeData = selectTreeNodeData;
        PageState.setSelectTreeNodeData = setSelectTreeNodeData;

        const [selectTreeNodeKeys, setSelectTreeNodeKeys] = React.useState([]);
        PageState.selectTreeNodeKeys = selectTreeNodeKeys;
        PageState.setSelectTreeNodeKeys = setSelectTreeNodeKeys;


        // 字典列表-查询数据
        const [searchFormData, setSearchFormData] = React.useState({});
        PageState.setSearchFormData = setSearchFormData;
        PageState.searchFormData = searchFormData;
        // 查询表单关联关系
        PageState.searchFormRef = React.useRef(null);

        // 选中表格key，选中表格行
        const [selectedRowKeys, setSelectedRowKeys] = React.useState([]);
        PageState.selectedRowKeys = selectedRowKeys;
        PageState.setSelectedRowKeys = setSelectedRowKeys;
        const [selectedRows, setSelectedRows] = React.useState([]);
        PageState.selectedRows = selectedRows;
        PageState.setSelectedRows = setSelectedRows;

        // 分页可选项
        const [showPageSize, setShowPageSize] = React.useState('default');
        PageState.showPageSize = showPageSize;
        PageState.setShowPageSize = setShowPageSize;
        // 列表数据
        const [gridData, setGridData] = React.useState([]);
        PageState.gridData = gridData;
        PageState.setGridData = setGridData;
        //分页数据
        const [pageData, setPageData] = React.useState({
            pageNo: 1,
            pageSize: 10,
            total: 0,
        });
        PageState.pageData = pageData;
        PageState.setPageData = setPageData;
    },
    findTreeData: (v: String) => {
        if(!v || v == ''){
            PageState.setSelectTreeNodeKeys([]);
            PageState.setSelectTreeNodeData({});
        }
        console.log(v);
        const treeData = [
            {
                dictName: '一级 1',
                dictCode: '0-0',
                dictChildren: [
                    {
                        dictName: '二级 1-0',
                        dictCode: '0-0-0',
                        dictChildren: [
                            {
                                dictName: '三级 1-0-0',
                                dictCode: '0-0-0-0',
                            },
                        ],
                    },
                ],
            },
        ]
        PageState.setTreeData(treeData);
    },
    findGridData: (searchFormData: any) => {
        const gridData = [
            {dictName: '中国', dictCode: '0-0',dictNamePath:'A/时代发生的发生地方/sdfsfsfd/斯蒂芬森该死的歌德换个地方好/,/sdfsfsfd/斯蒂芬森该死的歌德换个地方好/,/sdfsfsfd/斯蒂芬森该死的歌德换个地方好/' ,dictChildren: [{dictName: '二级 1-0', dictCode: '0-0-0', dictChildren: [{dictName: '三级 1-0-0', dictCode: '0-0-0-0',}]}]}
        ];
        PageState.setGridData(gridData);
    },
    addBrotherNode: (node: any) => {

        console.log('addDict')
    },
    addChildNode: (node) => {
        console.log('addDict')
    },
    editNode: (node) => {
        console.log('editDict')
    },
    removeNode: (node) => {
        console.log('delDict')
    },
    enableNode: (node) => {
        console.log('delDict')
    },
    disableNode: (node) => {
        console.log('delDict')
    },
    changeParentNode: (node) => {
        console.log('delDict')
    },
    changeOrderNode: (node) => {
        console.log('delDict')
    },
}
