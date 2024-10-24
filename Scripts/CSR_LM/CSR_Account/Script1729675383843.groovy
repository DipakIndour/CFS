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

WebUI.callTestCase(findTestCase('CSR_LM/CSRLM_Login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanDashboard/span_Account'))

WebUI.waitForElementClickable(findTestObject('CSR_AccountPage/Page_Loan Manager - LoanInfo/tab-heading_Terms'), 15)

WebUI.click(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanInfo/tab-heading_Terms'))

WebUI.click(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanInfo/em_Account Management_fa fa-pencil'))

WebUI.click(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanInfo/b'))

WebUI.setText(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanInfo/input_Control Level 1_loan_AM_17'), 
    '002')

WebUI.setText(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanInfo/input_Control Level 2_loan_AM_20'), 
    '003')

WebUI.click(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanInfo/em_Account Management_fa fa-save'))

WebUI.click(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanInfo/div_Your changes have been saved'))

WebUI.click(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanInfo/button_Account Management_loan_mgt'))

WebUI.click(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanInfo/span_1 - Joint'))

WebUI.click(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanInfo/em_Account Management_fa fa-save'))

WebUI.click(findTestObject('Object Repository/CSR_AccountPage/Page_Loan Manager - LoanInfo/button_OK'))

