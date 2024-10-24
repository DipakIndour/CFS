import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://toyotaqa.loanmanager.central.conduent.com/ReleaseTest/Admin/#/user/login')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Con_413165/input_Sign In_username'), 
    'Poorna')

WebUI.setEncryptedText(findTestObject('Object Repository/Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Con_413165/input_User name is required_password'), 
    'rVoIeBhAXzHLEVcHG9Ck2Q==')

WebUI.executeJavaScript('document.body.style.zoom=\'80%\'', null)

WebUI.click(findTestObject('Object Repository/Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Con_413165/input_Remember Me_remember'))

WebUI.click(findTestObject('Object Repository/Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Con_413165/Page_Conduent Business Services, LLC. - Conduent Loan Manager Client Management/button_Login'))

WebUI.click(findTestObject('Object Repository/Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Dashboard/span_Custom Dashboard'))

WebUI.click(findTestObject('Object Repository/Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Dashboard/span_Add'))

WebUI.verifyElementClickable(findTestObject('Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Add_a36d83/span_Select'))

WebUI.click(findTestObject('Object Repository/Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Add_a36d83/span_Select'))

WebUI.click(findTestObject('Object Repository/Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Add_a36d83/li_SuperUser'))

WebUI.waitForAngularLoad(240)

WebUI.verifyElementVisible(findTestObject('Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Add_a36d83/targetdropzoneA1'))

WebUI.verifyElementVisible(findTestObject('Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Add_a36d83/sourceDashboard1'))

WebUI.click(findTestObject('Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Add_a36d83/sourceDashboard1'))

WebUI.dragAndDropToObject(findTestObject('Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Add_a36d83/sourceDashboard1'), 
    findTestObject('Admin_LM_CustomDisplay/Page_Conduent Business Services, LLC. - Add_a36d83/targetdropzoneA1'))

WebUI.delay(40)

