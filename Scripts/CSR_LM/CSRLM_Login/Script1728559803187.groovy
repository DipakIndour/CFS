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

WebUI.navigateToUrl('https://toyotaqa.loanmanager.central.conduent.com/ReleaseTest/CSr/#/user/login')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/CSR_LM_Login/Page_Loan Manager - Conduent Client Loan Ma_68bd6f/input_Sign In_username'), 
    'Poorna')

WebUI.setEncryptedText(findTestObject('Object Repository/CSR_LM_Login/Page_Loan Manager - Conduent Client Loan Ma_68bd6f/input_User name is required_password'), 
    'rVoIeBhAXzHLEVcHG9Ck2Q==')

WebUI.click(findTestObject('Object Repository/CSR_LM_Login/Page_Loan Manager - Conduent Client Loan Ma_68bd6f/span_Remember Me_fa fa-check'))

WebUI.click(findTestObject('Object Repository/CSR_LM_Login/Page_Loan Manager - Conduent Client Loan Ma_68bd6f/button_Login'))

WebUI.click(findTestObject('CSR_LM_Login/Page_Loan Manager - Account Search/div_Account'))

WebUI.click(findTestObject('Object Repository/CSR_LM_Login/Page_Loan Manager - Account Search/span_Name'))

WebUI.setText(findTestObject('Object Repository/CSR_LM_Login/Page_Loan Manager - Account Search/input_First_lastname'), 
    'black')

WebUI.click(findTestObject('Object Repository/CSR_LM_Login/Page_Loan Manager - Account Search/button_Search'))

WebUI.click(findTestObject('Object Repository/CSR_LM_Login/Page_Loan Manager - Account Search/a_70000151884951001'))

/*if(WebUI.verifyAlertPresent(15)) {
	WebUI.comment("Alert is present")
	WebUI.getAlertText()
	WebUI.delay(5)
	WebUI.acceptAlert()
	}*/

